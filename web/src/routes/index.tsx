import { useQuery } from '@tanstack/react-query'
import { createFileRoute, Link } from '@tanstack/react-router'
import { useEffect } from 'react'

export const Route = createFileRoute('/')({ component: App })

type Ticket = {
  id: string,
  title: string,
  description: string,
  priority: string,
  status: string,
}

function App() {
  const { data } = useQuery<Ticket[]>({
      queryKey: ['tickets'],
      queryFn: () =>
        fetch('http://localhost:3001/api/tickets').then((res) =>
          res.json(),
        ),
    })

  return (
    <main className="px-4 pb-8 pt-14 flex flex-col justify-center items-center">
      <div className='flex gap-2'>
        <h2 >Tickets</h2>
        <Link to="/create" className='bg-blue-300 rounded-lg px-4 py-1'>Add</Link>
      </div>
      <table className='w-md'>
        <thead>
          <tr>
            <td>Title</td>
            <td>Status</td>
            <td>Priority</td>
            <td>Edit</td>
          </tr>
        </thead>
        <tbody>
          {data?.map(ticket => {
            return <tr key={ticket.id}>
              <td>{ticket.title}</td>
              <td>{ticket.status}</td>
              <td>{ticket.priority}</td>
              <td className='p-2'><Link to="/update/$ticketId" className='bg-blue-500 px-2 py-1 rounded-lg' params={{ ticketId: ticket.id}}>Update</Link></td>
            </tr>
          })}
        </tbody>
      </table>
    </main>
  )
}
