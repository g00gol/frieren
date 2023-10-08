import { RiStarSFill } from "react-icons/ri";
import moment from "moment";

export default function Card({ project }) {
  const languagesAndTechnology = [
    ...(project?.languages ?? []),
    ...(project?.technologies ?? []),
  ];

  const last_updated_human_readable = moment(project.last_updated).fromNow();

  return (
    <div className="w-full h-fit p-4 bg-deep-blue rounded-lg space-y-4">
      <div>
        <span className="flex w-full space-x-4">
          <a href={project.repo_origin}>{project.name}</a>
          <p className="flex items-center">
            <RiStarSFill color="#e3b341" /> {project.stars}
          </p>
        </span>
      </div>

      <div className="space-x-2">
        {languagesAndTechnology.map((language) => (
          <span key={language} className="badge p-3">
            <p className="mr-2">{language}</p>
          </span>
        ))}
      </div>

      <span className="flex w-full space-x-4">
        <p>Difficulty: {project.difficulty}/5</p>
        <p>-</p>
        {project.difficulty <= 3 ? (
          <p>
            <a
              href={`${
                project.repo_origin
              }/issues/?q=is:open+label:${project.recommended_issue_labels
                .map((label) => `"${label}"`)
                .join(",")}`}
            >
              Recommended issues
            </a>
            : {project.recommended_issues_count}
          </p>
        ) : (
          <></>
        )}
      </span>

      <p>{project.description}</p>

      <p>Last updated: {last_updated_human_readable}</p>
    </div>
  );
}
