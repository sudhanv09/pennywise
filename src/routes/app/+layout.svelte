<script lang="ts">
  import AppSidebar from "$lib/components/app/app-sidebar.svelte";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";

  let { children } = $props();
</script>

<Sidebar.Provider>
  <AppSidebar variant="inset" />
  <Sidebar.Inset>
    <header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
      <Sidebar.Trigger class="-ml-1" />
      <Separator orientation="vertical" class="mr-2 h-4" />
    </header>
    {@render children?.()}
  </Sidebar.Inset>
</Sidebar.Provider>
