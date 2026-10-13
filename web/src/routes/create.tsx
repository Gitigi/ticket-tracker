import { useMutation } from '@tanstack/react-query'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useState } from 'react'

export const Route = createFileRoute('/create')({
  component: RouteComponent,
})

type NewTicket = {
  title: string,
  description?: string,
  priority: string,
}

function RouteComponent() {
  const [newTicket, setTicket] = useState<NewTicket>({
    title: '',
    description: '',
    priority: 'Low'
  });

  const mutation = useMutation({
      mutationFn: (ticket: NewTicket) => {
      return fetch('http://localhost:3001/api/tickets', {
        method: 'POST',
        body: JSON.stringify(ticket),
        headers: {
          'Content-Type': 'application/json'
        }
      })
      },
    })

  const navigate = useNavigate();

  function onSubmit(e: any) {
    e.preventDefault()
    mutation.mutate(newTicket);
    navigate({to: "/"})
  }

  return <main className="px-4 pb-8 pt-14 flex flex-col justify-center items-center">
    <form onSubmit={onSubmit}>
      <div>
        <label htmlFor="title">Title</label>
        <input
          id="title" required type="text"
          className='ml-2 border-gray-300 border-2 rounded-md'
          onChange={(e) => {
            const title = e.target.value;
            setTicket(value => ({...value, title}))
          }}
        />
      </div>
      <div>
        <label htmlFor='description'>Description</label>
        <textarea
          id="description"
          className='ml-2 border-gray-300 border-2 rounded-md'
          onChange={(e) => {
            const description = e.target.value;
            setTicket(value => ({...value, description}))
          }}
        ></textarea>
      </div>
      <div>
        <label htmlFor='priority'>Priority</label>
        <select
          id="priority"
          className='ml-2 border-gray-300 border-2 rounded-md'
          onChange={(e) => {
            const priority = e.target.value;
            setTicket(value => ({...value, priority}))
          }}
        >
          <option>Low</option>
          <option>Medium</option>
          <option>High</option>
        </select>
      </div>
      <div>
        <button type="submit" className="bg-blue-200 p-2 rounded-lg">Submit</button>
      </div>
    </form>
  </main>
}
