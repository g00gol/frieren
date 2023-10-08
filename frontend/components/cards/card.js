export default function Card({ project }) {
  return (
    <div className="w-full h-fit p-4">
      <a href={project.repo_origin}>{project.name}</a>
    </div>
  );
}
