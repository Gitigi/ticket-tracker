import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/')({ component: App })

function App() {
  return (
    <main className="px-4 pb-8 pt-14 flex justify-center">
      <h2 >Tickets</h2>
      <ul>
        
      </ul>
    </main>
  )
}
