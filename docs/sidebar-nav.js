(() => {
  const localStorageKey = 'opencoven.docs.sidebarCollapsed';
  const mintlifyStorageKey = 'opencoven.docs.mintlifySidebarCollapsed';

  function readStorage(key) {
    try {
      return window.localStorage.getItem(key);
    } catch {
      return null;
    }
  }

  function writeStorage(key, collapsed) {
    try {
      window.localStorage.setItem(key, collapsed ? '1' : '0');
    } catch {
      // localStorage is optional; the buttons remain functional without it.
    }
  }

  function measuredWidth(selectors) {
    for (const selector of selectors) {
      const element = document.querySelector(selector);
      const width = element?.getBoundingClientRect().width ?? 0;
      if (width > 0) return width;
    }

    return 0;
  }

  function initLocalSidebar() {
    const sidebar = document.getElementById('docs-sidebar');
    const overlay = document.querySelector('[data-sidebar-overlay]');
    const triggers = Array.from(document.querySelectorAll('[data-sidebar-trigger]'));
    const closeButtons = Array.from(document.querySelectorAll('[data-sidebar-close]'));
    const desktopQuery = window.matchMedia('(min-width: 860px)');
    const spaciousQuery = window.matchMedia('(min-width: 1180px)');

    if (!sidebar || !overlay || triggers.length === 0) return;

    function isDesktop() {
      return desktopQuery.matches;
    }

    function setTriggerState(expanded) {
      for (const trigger of triggers) {
        trigger.setAttribute('aria-expanded', String(expanded));
      }
    }

    function setMobileOpen(open) {
      sidebar.dataset.open = String(open);
      overlay.hidden = !open;
      document.body.classList.toggle('sidebar-open', open);
      setTriggerState(open);
    }

    function shouldAutoCollapse() {
      if (!spaciousQuery.matches) return true;
      if (window.innerWidth >= 1460) return false;

      const contentWidth = measuredWidth(['.doc-content']);
      return contentWidth > 0 && contentWidth < 720;
    }

    function setDesktopCollapsed(collapsed, persist = false) {
      document.body.classList.toggle('sidebar-collapsed', collapsed);
      setTriggerState(!collapsed);
      if (persist) writeStorage(localStorageKey, collapsed);
    }

    function syncMode() {
      if (isDesktop()) {
        setMobileOpen(false);

        const stored = readStorage(localStorageKey);
        const collapsed = stored === '1' || (stored !== '0' && shouldAutoCollapse());
        setDesktopCollapsed(collapsed);
        return;
      }

      document.body.classList.remove('sidebar-collapsed');
      setMobileOpen(false);
    }

    function toggleSidebar() {
      if (isDesktop()) {
        const collapsed = !document.body.classList.contains('sidebar-collapsed');
        setDesktopCollapsed(collapsed, true);
        return;
      }

      setMobileOpen(sidebar.dataset.open !== 'true');
    }

    for (const trigger of triggers) {
      trigger.addEventListener('click', toggleSidebar);
    }

    for (const closeButton of closeButtons) {
      closeButton.addEventListener('click', () => setMobileOpen(false));
    }

    overlay.addEventListener('click', () => setMobileOpen(false));

    sidebar.addEventListener('click', (event) => {
      if (!isDesktop() && event.target instanceof HTMLAnchorElement) {
        setMobileOpen(false);
      }
    });

    document.addEventListener('keydown', (event) => {
      if (event.key === 'Escape' && !isDesktop() && sidebar.dataset.open === 'true') {
        setMobileOpen(false);
        triggers[0]?.focus();
      }
    });

    desktopQuery.addEventListener('change', syncMode);
    spaciousQuery.addEventListener('change', syncMode);
    window.addEventListener('resize', () => window.requestAnimationFrame(syncMode));
    window.requestAnimationFrame(syncMode);
  }

  function createSidebarTrigger() {
    const trigger = document.createElement('button');
    trigger.className = 'oc-sidebar-trigger';
    trigger.type = 'button';
    trigger.setAttribute('aria-label', 'Toggle docs navigation');
    trigger.setAttribute('aria-controls', 'sidebar');
    trigger.setAttribute('aria-expanded', 'true');
    trigger.setAttribute('data-oc-mintlify-sidebar-trigger', '');
    trigger.innerHTML =
      '<span class="oc-sidebar-trigger-icon" aria-hidden="true"><span></span><span></span><span></span></span>';
    return trigger;
  }

  function initMintlifySidebar() {
    const sidebar = document.getElementById('sidebar');
    const navbar = document.getElementById('navbar');
    if (!sidebar || !navbar) return false;

    let trigger = document.querySelector('[data-oc-mintlify-sidebar-trigger]');
    if (!trigger) {
      trigger = createSidebarTrigger();
      const logo = navbar.querySelector('nav-logo') || navbar.querySelector('a[href="/"]');
      if (logo?.parentElement) {
        logo.parentElement.insertBefore(trigger, logo);
      } else {
        navbar.prepend(trigger);
      }
    }

    const desktopQuery = window.matchMedia('(min-width: 1024px)');
    const spaciousQuery = window.matchMedia('(min-width: 1320px)');

    function shouldAutoCollapse() {
      if (!desktopQuery.matches) return false;
      if (!spaciousQuery.matches) return true;
      if (window.innerWidth >= 1540) return false;

      const contentWidth = measuredWidth(['#content-area', '#content', 'mdx-content']);
      return contentWidth > 0 && contentWidth < 720;
    }

    function setCollapsed(collapsed, persist = false) {
      document.body.classList.toggle('oc-sidebar-collapsed', collapsed);
      trigger.setAttribute('aria-expanded', String(!collapsed));
      if (persist) writeStorage(mintlifyStorageKey, collapsed);
    }

    function syncMode() {
      if (!desktopQuery.matches) {
        document.body.classList.remove('oc-sidebar-collapsed');
        trigger.setAttribute('aria-expanded', 'false');
        return;
      }

      const stored = readStorage(mintlifyStorageKey);
      const collapsed = stored === '1' || (stored !== '0' && shouldAutoCollapse());
      setCollapsed(collapsed);
    }

    trigger.addEventListener('click', () => {
      const collapsed = !document.body.classList.contains('oc-sidebar-collapsed');
      setCollapsed(collapsed, true);
    });

    desktopQuery.addEventListener('change', syncMode);
    spaciousQuery.addEventListener('change', syncMode);
    window.addEventListener('resize', () => window.requestAnimationFrame(syncMode));
    window.requestAnimationFrame(syncMode);

    return true;
  }

  function initWhenReady() {
    initLocalSidebar();

    let attempts = 0;
    const tryMintlify = () => {
      attempts += 1;
      if (initMintlifySidebar() || attempts >= 20) return;
      window.setTimeout(tryMintlify, 150);
    };

    tryMintlify();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initWhenReady, { once: true });
  } else {
    initWhenReady();
  }
})();
