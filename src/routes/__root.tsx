import { Outlet, createRootRouteWithContext } from '@tanstack/solid-router'
import { QueryClient, QueryClientProvider } from '@tanstack/solid-query'

const queryCLient = new QueryClient();

import Header from '../components/Header'

export const Route = createRootRouteWithContext()({
  component: RootComponent,
})

function RootComponent() {
  return (
    <>
      <QueryClientProvider client={queryCLient}>
        <Header />

        <Outlet />
      </QueryClientProvider>
    </>
  )
}
