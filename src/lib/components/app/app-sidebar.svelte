<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import type { ComponentProps } from "svelte";

  import House from "@lucide/svelte/icons/house";
  import SquareActivity from "@lucide/svelte/icons/square-activity";
  import Goal from "@lucide/svelte/icons/goal";
  import Landmark from "@lucide/svelte/icons/landmark";
  import Handcoins from "@lucide/svelte/icons/hand-coins";
  import CalendarCheck2 from "@lucide/svelte/icons/calendar-check-2";
  import CirclePoundSterling from "@lucide/svelte/icons/circle-pound-sterling";
  import Settings from "@lucide/svelte/icons/settings";
  import LogOut from "@lucide/svelte/icons/log-out";

  const data = {
    main: [
      {
        title: "Dashboard",
        url: "/app",
        icon: House,
      },
      {
        title: "Activity",
        url: "/app/activity",
        icon: SquareActivity,
      },
    ],
    user: [
      {
        title: "Accounts",
        url: "/app/accounts",
        icon: Landmark,
      },
      {
        title: "Goals",
        url: "/app/goals",
        icon: Goal,
      },
      {
        title: "Loan",
        url: "/app/loans",
        icon: Handcoins,
      },
      {
        title: "Subscriptions",
        url: "/app/subscriptions",
        icon: CalendarCheck2,
      },
    ],
    footer: [
      {
        title: "Settings",
        url: "/settings",
        icon: Settings,
      },
      {
        title: "Logout",
        url: "#",
        icon: LogOut,
      },
    ],
  };
  let { ...restProps }: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root collapsible="offcanvas" {...restProps}>
  <Sidebar.Header>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton class="data-[slot=sidebar-menu-button]:!p-1.5">
          {#snippet child({ props })}
            <a href="##" {...props}>
              <CirclePoundSterling class="!size-5" />
              <span class="text-base font-semibold">Pennywise</span>
            </a>
          {/snippet}
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Header>
  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.GroupContent class="flex flex-col gap-2">
        <Sidebar.Menu>
          {#each data.main as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton tooltipContent={item.title}>
                {#snippet child({ props })}
                  <a href={item.url} {...props}>
                    <item.icon />
                    <span>{item.title}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
    <Sidebar.Group>
      <Sidebar.GroupLabel>User</Sidebar.GroupLabel>
      <Sidebar.GroupContent class="flex flex-col gap-2">
        <Sidebar.Menu>
          {#each data.user as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton tooltipContent={item.title}>
                {#snippet child({ props })}
                  <a href={item.url} {...props}>
                    <item.icon />
                    <span>{item.title}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>
  <Sidebar.Footer>
    <Sidebar.Menu>
      {#each data.footer as item (item.title)}
        <Sidebar.MenuItem>
          <Sidebar.MenuButton tooltipContent={item.title}>
            {#snippet child({ props })}
              <a href={item.url} {...props}>
                <item.icon />
                <span>{item.title}</span>
              </a>
            {/snippet}
          </Sidebar.MenuButton>
        </Sidebar.MenuItem>
      {/each}
    </Sidebar.Menu>
  </Sidebar.Footer>
</Sidebar.Root>
