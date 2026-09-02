/* MineRStatus docs — shared scripts and i18n core */

(function () {
  "use strict";

  var LS_KEY = "minestatus-docs-lang";

  /* ------------------------------------------------------------------ */
  /* Language                                                           */
  /* ------------------------------------------------------------------ */

  function availableLangs() {
    return Object.keys(window.UI || {});
  }

  function normalizeLang(code) {
    if (code && availableLangs().indexOf(code) !== -1) return code;
    return null;
  }

  function currentLang() {
    var p = queryParams();
    var fromUrl = normalizeLang(p.lang);
    if (fromUrl) return fromUrl;

    var stored = null;
    try {
      stored = localStorage.getItem(LS_KEY);
    } catch (e) {
      stored = null;
    }
    var fromStorage = normalizeLang(stored);
    if (fromStorage) return fromStorage;

    var nav = (navigator.language || "en").toLowerCase();
    if (nav.indexOf("zh") === 0) return normalizeLang("zh") || "en";
    return normalizeLang("en") || availableLangs()[0] || "en";
  }

  /**
   * Translate a dotted key with fallback to English (then the key itself).
   * Values may be strings, arrays or objects. `{param}` placeholders in
   * strings are replaced from the `params` argument.
   */
  function t(key, params) {
    var lang = currentLang();
    var val = undefined;
    if (window.UI && window.UI[lang]) val = window.UI[lang][key];
    if (val === undefined && window.UI && window.UI.en) {
      val = window.UI.en[key];
    }
    if (val === undefined) val = key;
    if (typeof val === "string" && params) {
      for (var k in params) {
        val = val.replace(new RegExp("\\{" + k + "\\}", "g"), params[k]);
      }
    }
    return val;
  }

  function switchLang(code) {
    try {
      localStorage.setItem(LS_KEY, code);
    } catch (e) {
      /* ignore */
    }
    var u = new URL(window.location.href);
    u.searchParams.set("lang", code);
    window.location.href = u.toString();
  }

  /* ------------------------------------------------------------------ */
  /* DOM helpers                                                        */
  /* ------------------------------------------------------------------ */

  function queryParams() {
    var out = {};
    new URLSearchParams(window.location.search).forEach(function (v, k) {
      out[k] = v;
    });
    return out;
  }

  function escapeHtml(s) {
    return String(s)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

  function syntaxHighlight(json) {
    var text = JSON.stringify(json, null, 2);
    text = escapeHtml(text);
    return text.replace(
      /("(\\u[a-fA-F0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+-]?\d+)?)/g,
      function (match) {
        var cls = "num";
        if (/^"/.test(match)) {
          cls = /:$/.test(match) ? "key" : "str";
        } else if (/true|false/.test(match)) {
          cls = "bool";
        } else if (/null/.test(match)) {
          cls = "null";
        }
        return '<span class="hl-' + cls + '">' + match + "</span>";
      }
    );
  }

  /* ------------------------------------------------------------------ */
  /* Nav + i18n application                                             */
  /* ------------------------------------------------------------------ */

  function currentPage() {
    var seg = window.location.pathname.split("/").pop() || "";
    return seg ? seg : "index.html";
  }

  function injectNav() {
    if (document.getElementById("nav-root")) return;

    var page = currentPage();
    var lang = currentLang();

    var links = [
      ["index.html", t("nav.home")],
      ["wiki.html", t("nav.wiki")],
      ["api-test.html", t("nav.apiTest")],
    ];

    var html =
      '<nav class="nav"><a class="brand" href="index.html">MineRStatus</a><div class="links">';
    links.forEach(function (l) {
      var active = l[0] === page ? ' class="active"' : "";
      html += '<a href="' + l[0] + '"' + active + ">" + l[1] + "</a>";
    });
    html += "</div>";

    // Language selector, built from the loaded UI bundles.
    var select =
      '<select class="lang-switch" id="lang-switch" title="Language">';
    availableLangs().forEach(function (code) {
      var name = (window.UI[code] && window.UI[code].name) || code;
      var enName = (window.UI.en && window.UI.en.name) || code;
      if (!name) name = enName;
      var selected = code === lang ? " selected" : "";
      select +=
        '<option value="' + code + '"' + selected + ">" + name + "</option>";
    });
    select += "</select>";
    html += select + "</nav>";

    document.body.insertAdjacentHTML("afterbegin", html);

    var sw = document.getElementById("lang-switch");
    if (sw) {
      sw.addEventListener("change", function () {
        if (sw.value !== lang) switchLang(sw.value);
      });
    }
  }

  /** Apply data-i18n* attributes to the DOM for the current language. */
  function applyI18n() {
    document.querySelectorAll("[data-i18n]").forEach(function (el) {
      var val = t(el.getAttribute("data-i18n"));
      if (typeof val === "string") el.textContent = val;
    });
    document.querySelectorAll("[data-i18n-placeholder]").forEach(function (el) {
      var val = t(el.getAttribute("data-i18n-placeholder"));
      if (typeof val === "string") el.setAttribute("placeholder", val);
    });
    document.querySelectorAll("[data-i18n-title]").forEach(function (el) {
      var val = t(el.getAttribute("data-i18n-title"));
      if (typeof val === "string") el.setAttribute("title", val);
    });
    document.querySelectorAll("[data-i18n-doc-title]").forEach(function (el) {
      var val = t(el.getAttribute("data-i18n-doc-title"));
      if (typeof val === "string") document.title = val;
    });
  }

  /**
   * API base URL. Defaults to the public deployment; override with
   * `window.MINESTATUS_API` (set before app.js loads) or a `?api=` query
   * parameter, e.g. `?api=https://api.example.com`. Trailing slashes are
   * stripped so endpoint paths (/, /java, /bedrock) append cleanly.
   */
  var DEFAULT_API = "https://v3.mscpo.giize.com/";

  function apiBase() {
    var base;
    if (window.MINESTATUS_API) {
      base = window.MINESTATUS_API;
    } else {
      var p = queryParams();
      base = p.api || DEFAULT_API;
    }
    return base.replace(/\/+$/, "");
  }

  /**
   * Render a server status result as a blueprint panel:
   * server icon on the left, host/version/status/motd on the right.
   * All dynamic text is escaped; the icon is only used as an <img> when it
   * is a PNG/JPEG/GIF/WebP data URI.
   */
  function serverResultHTML(data, host) {
    var ok = !!(data && data.online === true);
    var icon = (data && data.icon) || "";
    var img;
    if (/^data:image\/(png|jpe?g|gif|webp);base64,/i.test(icon)) {
      img = '<img src="' + icon + '" alt="server icon" />';
    } else {
      img =
        '<span class="srv-ni">' + (ok ? "NO ICON" : "OFFLINE") + "</span>";
    }

    var ver = data && data.version ? escapeHtml(data.version) : "";
    var delay = data && data.delay != null ? data.delay + " ms" : "";
    var players =
      data && data.players
        ? data.players.online + "/" + data.players.max
        : "";
    var motd = "";
    if (data && data.motd) {
      motd = escapeHtml(data.motd.plain || data.motd.minecraft || "");
    }
    var err = "";
    if (!ok && data && data.error) err = escapeHtml(data.error);

    var status = ok
      ? '<span class="pill ok">' + escapeHtml(t("index.statusOnline")) + "</span>"
      : '<span class="pill err">' + escapeHtml(t("index.statusOffline")) + "</span>";

    return (
      '<div class="srv">' +
      '<div class="srv-icon">' + img + "</div>" +
      '<div class="srv-info">' +
      '<div class="srv-head">' +
      '<span class="srv-host">' + escapeHtml(host) + "</span>" +
      (ver ? '<span class="srv-ver">' + ver + "</span>" : "") +
      status +
      (delay ? '<span class="pill">' + delay + "</span>" : "") +
      (players ? '<span class="pill">' + players + "</span>" : "") +
      "</div>" +
      (motd ? '<div class="srv-motd">' + motd + "</div>" : "") +
      (err ? '<div class="srv-error">' + err + "</div>" : "") +
      "</div>" +
      "</div>"
    );
  }

  window.Docs = {
    currentLang: currentLang,
    availableLangs: availableLangs,
    t: t,
    switchLang: switchLang,
    queryParams: queryParams,
    apiBase: apiBase,
    escapeHtml: escapeHtml,
    syntaxHighlight: syntaxHighlight,
    serverResultHTML: serverResultHTML,
    applyI18n: applyI18n,
  };

  function boot() {
    injectNav();
    applyI18n();
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", boot);
  } else {
    boot();
  }
})();