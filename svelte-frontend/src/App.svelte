<script lang="ts">
  import {
    Content,
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
  } from "carbon-icons-svelte";

  import IdePage from "./pages/IdePage.svelte";
  import HomePage from "./pages/HomePage.svelte";
  import PlaceholderPage from "./pages/PlaceholderPage.svelte";

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
  ];

  function toggleMenu() {
    isSideNavOpen = !isSideNavOpen;
  }

  // Custom navigation function that updates history and current path
  function navigateTo(path: string): void {
    if (typeof window !== "undefined") {
      window.history.pushState({}, "", path);
      currentPath = path;
    }
  }
</script>

<div class="app-container">
  <div class="top-spacing"></div>
  <Content>
    <Header platformName="" class="fancy-header">
      <div slot="skip-to-content">
        <SkipToContent />
      </div>
      <div class="custom-header">
        <div class="hamburger-container">
          <button
            class="hamburger"
            on:click={toggleMenu}
            aria-label="Toggle menu"
            aria-controls="sidebar-menu"
          >
            <div class={`hamburger-icon ${isSideNavOpen ? "open" : ""}`}>
              <span></span>
              <span></span>
              <span></span>
            </div>
          </button>
        </div>
        <div class="app-title">NFe Management</div>
        <div class="header-decoration-dot"></div>
      </div>
    </Header>

    <SideNav id="sidebar-menu" bind:isOpen={isSideNavOpen}>
      <SideNavItems>
        {#each menuItems as item}
          <div class="side-nav-link-wrapper">
            <SideNavLink
              icon={item.icon}
              text={item.name}
              isSelected={currentPath === item.path}
              on:click={() => navigateTo(item.path)}
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
      {/if}
    </div>
  </Content>
</div>
