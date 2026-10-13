import { useMutation } from '@tanstack/react-query';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useState } from 'react'

export const Route = createFileRoute('/update/$ticketId')({
  component: RouteComponent,
  loader: async ({ params }) => {
    return fetch(`http://localhost:3001/api/tickets/${params.ticketId}`).then(res => {
        return res.json()
      })
    },
})

function RouteComponent() {
  const { ticketId } = Route.useParams()
  const ticket = Route.useLoaderData()
  const [status, setStatus] = useState(ticket.status);

  const mutation = useMutation({
      mutationFn: (status: string) => {
        return fetch(`http://localhost:3001/api/tickets/${ticketId}`, {
        method: 'PATCH',
        body: JSON.stringify({status}),
        headers: {
          'Content-Type': 'application/json'
        }
      })
      },
    })

  const navigate = useNavigate();

  function onSubmit(e: any) {
    e.preventDefault()
    mutation.mutate(status);
    navigate({to: "/"})
  }

  return <main className="px-4 pb-8 pt-14 flex flex-col justify-center items-center">
    <form onSubmit={onSubmit}>
      <div>
        <label htmlFor='status'>Status</label>
        <select
          id="status"
          className='ml-2 border-gray-300 border-2 rounded-md'
          value={status}
          onChange={(e) => {
            setStatus(e.target.value)
          }}
        >
          <option>Open</option>
          <option>InProgress</option>
          <option>Closed</option>
        </select>
      </div>
      <div>
        <button type="submit" className="bg-blue-200 p-2 rounded-lg">Submit</button>
      </div>
    </form>
  </main>
}
