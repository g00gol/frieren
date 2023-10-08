import Link from "next/link";

export default function Nav() {
  return (
    <nav className="flex my-12">
      <Link href="/">
        <h3>frieren.playground</h3>
      </Link>
    </nav>
  );
}
