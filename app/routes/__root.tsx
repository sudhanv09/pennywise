import {
  Outlet,
  createRootRoute,
} from '@tanstack/solid-router'
import { Sidebar } from '~/components/sidebar'

export const Route = createRootRoute({
  head: () => ({
    meta: [
      {
        charset: 'utf-8',
      },
      {
        name: 'viewport',
        content: 'width=device-width, initial-scale=1',
      },
      {
        title: 'Pennywise',
      },
    ],
  }),
  component: RootComponent,
})

function RootComponent() {
  return (
    <div class='app'>
      <Sidebar />
      <Outlet />
    </div>)
}