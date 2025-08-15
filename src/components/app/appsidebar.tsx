import { For } from "solid-js"
import type { ComponentProps } from "solid-js"
import { A } from "@solidjs/router"

import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail
} from "~/components/ui/sidebar"

const data = {
  navMain: [
    {
      title: "Getting Started",
      url: "#",
      items: [
        {
          title: "Installation",
          url: "#"
        },
        {
          title: "Project Structure",
          url: "#"
        }
      ]
    },
    {
      title: "Building Your Application",
      url: "#",
      items: [
        {
          title: "Routing",
          url: "#"
        },
        {
          title: "Data Fetching",
          url: "#",
          isActive: true
        },
        {
          title: "Rendering",
          url: "#"
        },
      ]
    }
  ]
}

export function AppSidebar(props: ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar {...props}>
      <SidebarHeader class="text-center p-4">
        <h1 class="text-xl">Pennywise</h1>
      </SidebarHeader>
      <SidebarContent>
        <For each={data.navMain}>
          {(item) => (
            <SidebarGroup>
              <SidebarGroupContent>
                <SidebarMenu>
                  <For each={item.items}>
                    {(item) => (
                      <SidebarMenuItem>
                        <SidebarMenuButton as={A} isActive={item.isActive} href={item.url}>
                          {item.title}
                        </SidebarMenuButton>
                      </SidebarMenuItem>
                    )}
                  </For>
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          )}
        </For>
      </SidebarContent>
      <SidebarRail />
    </Sidebar>
  )
}