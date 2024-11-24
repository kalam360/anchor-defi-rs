## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

## Course Info
- [Github](https://github.com/adrianhajdin/yc_directory)
- [Youtube](https://www.youtube.com/watch?v=Zq5fmkH0T78)
- [Project Readme](https://github.com/adrianhajdin/yc_directory/blob/main/README.md)

## Topics to understand Nextjs
### Server and Client Components in Next.js

In Next.js, components can be designated as either server or client components. This distinction is crucial for performance optimization and functionality separation.

#### Server Components
Server components are rendered on the server-side and then sent to the client as static HTML. They do not have direct access to browser-specific APIs like `window` or `document`. Since they run only on the server, they can be used for data fetching, authentication checks, and any other operations that should happen before sending content to the client.

**Example of a Server Component:**

```tsx
// app/data-fetching/page.tsx

export default async function DataFetchingPage() {
  const response = await fetch('https://api.example.com/data');
  const data = await response.json();

  return (
    <div>
      <h1>Data from API</h1>
      <pre>{JSON.stringify(data, null, 2)}</pre>
    </div>
  );
}
```
#### Client Components
Client components are rendered on the client-side and have full access to browser-specific APIs. They can be used for interactive elements such as forms, event handlers, and state management.

**Example of a Client Component:**

```tsx
// app/interactive-component/page.tsx

'use client';

import { useState } from 'react';

export default function InteractiveComponent() {
  const [count, setCount] = useState(0);

  return (
    <div>
      <h1>Counter</h1>
      <p>You clicked {count} times</p>
      <button onClick={() => setCount(count + 1)}>
        Click me
      </button>
    </div>
  );
}
```

### Server Component HMR cache

Hot Module Replacement (HMR) is a feature in Next.js that allows you to update modules in the browser at runtime without a full refresh. When working with server components, understanding how HMR interacts with the cache is essential for an optimal development experience.

During development, when you make changes to a server component, Next.js will attempt to preserve the state and re-render only the affected parts of your application. This process involves caching the results of server-side rendering to avoid unnecessary data fetching or computation on every change.

However, there are scenarios where HMR might not work as expected with server components:
1. **Data Fetching:** If a server component fetches data from an external API, changes in the component's logic that affect data fetching can lead to full page reloads rather than hot module replacement.
2. **State Management:** Server components do not have their own state since they are rendered on the server side. Any state management must be handled by client components or through other means like URL parameters or query strings.

To ensure efficient HMR with server components, it's important to keep them focused on data fetching and rendering static HTML. Interactive logic should be delegated to client components where HMR can handle updates more gracefully.

**Example of Efficient HMR Usage:**

In the example below, we separate data fetching (server component) from interactive logic (client component):

```tsx
// app/data-fetching/page.tsx

export default async function DataFetchingPage() {
  const response = await fetch('https://api.example.com/data');
  const data = await response.json();

  return (
    <div>
      <h1>Data from API</h1>
      <pre>{JSON.stringify(data, null, 2)}</pre>
      <InteractiveComponent initialCount={data.initialCount} />
    </div>
  );
}

// app/interactive-component/page.tsx

'use client';

import { useState } from 'react';

export default function InteractiveComponent({ initialCount }) {
  const [count, setCount] = useState(initialCount);

  return (
    <div>
      <h1>Counter</h1>
      <p>You clicked {count} times</p>
      <button onClick={() => setCount(count + 1)}>
        Click me
      </button>
    </div>
  );
}
```
### Static Site Generation and Incremental Static Regeneration
#### Static site Generation
#### Incremental Static Regeneration
- Time based revalidation
- On-demand revalidation
### Server Side Rendering
### Partial Prerendering
### Metadata and generateMetadata in Nextjs
### Use Action State Hook
### Nextjs Server Action

### Nextjs Auth
### Server Form action react 19 features



## Additional Tools
### [Tailwindcss](https://tailwindcss.com/)
### [Shadcn](https://ui.shadcn.com/)
### NextAuth
### [NextSanity](https://github.com/sanity-io/next-sanity)


