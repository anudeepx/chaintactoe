'use client'

import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactNode, useState } from 'react'

export function QueryProvider({ children }: { children: ReactNode }) {
  const [client] = useState(() => new QueryClient())

  return <QueryClientProvider client={client}>{children}</QueryClientProvider>
}

// usuage
// app/page.tsx or any component
// 'use client'

// import { useQuery } from '@tanstack/react-query'
// import axios from 'axios'

// const fetchData = async () => {
//   const res = await axios.get('https://api.example.com/data')
//   return res.data
// }

// export default function HomePage() {
//   const { data, isLoading, error } = useQuery({
//     queryKey: ['my-data'],
//     queryFn: fetchData,
//   })

//   if (isLoading) return <p>Loading...</p>
//   if (error) return <p>Error loading data</p>

//   return <pre>{JSON.stringify(data, null, 2)}</pre>
// }
