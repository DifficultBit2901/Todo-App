import { createBrowserRouter, RouterProvider } from "react-router-dom"
import Layout from "../components/views/Layout.tsx"
const routes = [
  {
    path: "/",
    element: <Layout />,
    children: [

    ]
  }
];

const router = createBrowserRouter(routes);

export default function Router() {
  return <RouterProvider router={router} />
}
