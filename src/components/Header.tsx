import { Link } from "@tanstack/solid-router";

export default function Header() {
  return (
    <header>
      <nav>
        <div>
          <Link to="/">Home</Link>
        </div>

        <div>
          <Link to="/demo/start/server-funcs">Start - Server Functions</Link>
        </div>

        <div>
          <Link to="/demo/form">Form</Link>
        </div>

        <div>
          <Link to="/demo/tanstack-query">TanStack Query</Link>
        </div>
      </nav>
    </header>
  );
}
