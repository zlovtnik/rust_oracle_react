<script lang="ts">
  import {
    Header,
    SideNav,
    SideNavItems,
    SideNavLink,
    SkipToContent,
  } from "carbon-components-svelte";
  import {
    UserProfile,
    Enterprise,
    User,
    Calculator,
    DeliveryTruck,
    Money,
    Wallet,
    Information,
    ChartBar,
  } from "carbon-icons-svelte";

  import IdePage from "./pages/IdePage.svelte";
  import HomePage from "./pages/HomePage.svelte";
  import PlaceholderPage from "./pages/PlaceholderPage.svelte";
  import MetricsPage from "./pages/MetricsPage.svelte";

  let isSideNavOpen = false;
  let currentPath = "/";

  // Initialize the current path based on the window location
  if (typeof window !== "undefined") {
    currentPath = window.location.pathname || "/";

    // Handle history navigation
    window.addEventListener("popstate", () => {
      currentPath = window.location.pathname || "/";
    });
  }

  // Define menu items for navigation
  const menuItems = [
    { path: "/", name: "Dashboard", icon: UserProfile },
    { path: "/ide", name: "Identification (ide)", icon: UserProfile },
    { path: "/emit", name: "Issuer (emit)", icon: Enterprise },
    { path: "/dest", name: "Recipient (dest)", icon: User },
    { path: "/total", name: "Totals (total)", icon: Calculator },
    { path: "/transp", name: "Transport (transp)", icon: DeliveryTruck },
    { path: "/cobr", name: "Billing (cobr)", icon: Money },
    { path: "/pag", name: "Payment (pag)", icon: Wallet },
    { path: "/infAdic", name: "Additional Info (infAdic)", icon: Information },
    { path: "/metrics", name: "System Metrics", icon: ChartBar },
  ];

  // Custom navigation function that updates history and current path
  function navigateTo(path: string): void {
    if (typeof window !== "undefined") {
      window.history.pushState({}, "", path);
      currentPath = path;
    }
  }

  // Force menu to be visible at all times
  $: if (typeof window !== "undefined") {
    const menuTrigger = document.querySelector(
      ".bx--header__menu-trigger",
    ) as HTMLElement;
    if (menuTrigger) {
      menuTrigger.classList.remove("bx--header__menu-trigger--hidden");
      menuTrigger.setAttribute("aria-hidden", "false");
      menuTrigger.style.display = "block";
      menuTrigger.style.visibility = "visible";
      menuTrigger.style.opacity = "1";
    }
  }
</script>

<div class="app-container">
  <div class="top-spacing"></div>
  <Header
    platformName="NFe Management"
    class="header-custom"
    bind:isSideNavOpen
  >
    <div slot="skip-to-content">
      <SkipToContent />
    </div>
  </Header>

  <div class="main-layout">
    <SideNav id="sidebar-menu" bind:isOpen={isSideNavOpen}>
      <SideNavItems>
        {#each menuItems as item}
          <div class="side-nav-link-wrapper">
            <SideNavLink
              icon={item.icon}
              text={item.name}
              isSelected={currentPath === item.path}
              on:click={(e) => {
                e.preventDefault();
                navigateTo(item.path);
                isSideNavOpen = false;
              }}
              class="nav-link"
            />
          </div>
        {/each}
      </SideNavItems>
    </SideNav>

    <div class="content-container">
      {#if currentPath === "/"}
        <HomePage {navigateTo} />
      {:else if currentPath === "/ide"}
        <IdePage {navigateTo} />
      {:else if currentPath === "/emit"}
        <PlaceholderPage title="Issuer (emit)" />
      {:else if currentPath === "/dest"}
        <PlaceholderPage title="Recipient (dest)" />
      {:else if currentPath === "/total"}
        <PlaceholderPage title="Totals (total)" />
      {:else if currentPath === "/transp"}
        <PlaceholderPage title="Transport (transp)" />
      {:else if currentPath === "/cobr"}
        <PlaceholderPage title="Billing (cobr)" />
      {:else if currentPath === "/pag"}
        <PlaceholderPage title="Payment (pag)" />
      {:else if currentPath === "/infAdic"}
        <PlaceholderPage title="Additional Information" />
      {:else if currentPath === "/metrics"}
        <MetricsPage />
      {/if}
    </div>
  </div>
</div>

<style>
  /* Define a modern colour palette */
  :root {
    --primary: #2a9d8f;
    --secondary: #264653;
    --accent: #e9c46a;
    --background: #121212;
    --text: #333;
  }

  .app-container {
    background-color: var(--background);
    color: var(--text);
    min-height: 100vh;
    width: 100%;
    overflow-x: hidden;
  }

  /* Use :global() for the Header component class */
  :global(.header-custom) {
    background-color: #004431 !important;
    color: #ffd700;
  }

  /* New main layout styles */
  .main-layout {
    display: flex;
    min-height: 100vh;
    background-color: var(--background);
  }

  .content-container {
    flex: 1;
    padding: 20px;
    margin-left: 48px;
    background-color: var(--background);
    min-height: 100vh;
  }

  /* Use :global() for the SideNavLink component classes */
  :global(.nav-link) {
    position: relative;
    transition: color 0.3s ease;
  }

  :global(.nav-link::before) {
    content: "";
    position: absolute;
    left: -10px;
    top: 50%;
    transform: translateY(-50%);
    width: 6px;
    height: 6px;
    background: #ffd700;
    border-radius: 50%;
    opacity: 0;
    transition:
      opacity 0.6s ease,
      transform 0.6s ease;
  }

  :global(.nav-link:hover::before) {
    opacity: 1;
    transform: translateY(-50%) scale(1.5);
  }

  /* Fix for Carbon components */
  :global(.bx--side-nav) {
    position: fixed;
    top: 0;
    left: 0;
    height: 100vh;
    background-color: #1e1e1e !important;
    border-right: 1px solid #333333;
    z-index: 8000;
  }

  :global(.bx--side-nav__items) {
    padding-top: 48px;
  }

  :global(.bx--header) {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    z-index: 9000;
    background-color: #004431 !important;
    padding-left: 0 !important;
    display: flex !important;
    align-items: center !important;
  }

  /* Consolidated menu trigger styles */
  :global(.bx--header__menu-trigger),
  :global(.bx--header__menu-trigger.bx--header__menu-trigger--hidden),
  :global(.bx--header__menu-trigger[aria-hidden="true"]),
  :global(.bx--header__menu-trigger[aria-expanded="false"]),
  :global(.bx--header__menu-trigger[aria-expanded="true"]),
  :global(.bx--header__menu-trigger[class*="hidden"]),
  :global(.bx--header__menu-trigger[class*="bx--header__menu-trigger--hidden"]),
  :global([class*="bx--header__menu-trigger"]) {
    position: fixed !important;
    left: 0 !important;
    top: 0 !important;
    width: 48px !important;
    height: 48px !important;
    background-color: transparent !important;
    border: none !important;
    z-index: 9500 !important;
    display: block !important;
    visibility: visible !important;
    opacity: 1 !important;
    pointer-events: auto !important;
    margin: 0 !important;
    padding: 0 !important;
    cursor: pointer !important;
    transform: none !important;
    transition: none !important;
  }

  :global(.bx--header__menu-trigger > svg) {
    fill: #ffd700 !important;
    width: 20px !important;
    height: 20px !important;
    margin: 14px !important;
    display: block !important;
    visibility: visible !important;
    opacity: 1 !important;
    transform: none !important;
    transition: none !important;
  }

  :global(.bx--header__menu-trigger:hover > svg) {
    fill: #ffeb99 !important;
  }

  :global(.bx--header__menu-trigger:focus) {
    outline: 2px solid #ffd700 !important;
    outline-offset: -2px !important;
  }

  :global(.bx--header__menu-trigger:focus > svg) {
    fill: #ffeb99 !important;
  }

  :global(.bx--header__name) {
    margin-left: 48px !important;
    padding-left: 0 !important;
    display: flex !important;
    align-items: center !important;
  }

  :global(.bx--header__global) {
    margin-left: 0 !important;
    display: flex !important;
    align-items: center !important;
  }

  :global(.bx--header__action) {
    display: flex !important;
    align-items: center !important;
  }

  :global(#main-content) {
    margin-top: 48px;
    background-color: #121212;
    min-height: calc(100vh - 48px);
  }

  :global(.bx--side-nav__link) {
    color: #ffd700 !important;
  }

  :global(.bx--side-nav__link:hover) {
    background-color: rgba(255, 215, 0, 0.1) !important;
    color: #ffeb99 !important;
  }

  :global(.bx--side-nav__link--current) {
    background-color: #004431 !important;
  }

  /* Override Carbon's media queries */
  @media (min-width: 66rem) {
    :global(.bx--header__menu-trigger) {
      display: block !important;
      visibility: visible !important;
      opacity: 1 !important;
      pointer-events: auto !important;
      position: fixed !important;
      left: 0 !important;
      top: 0 !important;
    }
  }
</style>
