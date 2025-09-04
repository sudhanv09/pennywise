import { createFileRoute, Navigate } from '@tanstack/solid-router'

export const Route = createFileRoute('/')({
  component: IndexComponent,
})

function IndexComponent() {
  return <Navigate to="/dashboard" />
}
