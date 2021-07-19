import { hasProperty } from "../utils";

// eslint-disable-next-line import/no-default-export
export default interface MergedProblem {
  // Basic information
  readonly id: string;
  readonly contest_id: string;
  readonly title: string;

  // Information for first AC
  readonly first_user_id: string | null;
  readonly first_contest_id: string | null;
  readonly first_submission_id: number | null;

  // Information for fastest code
  readonly fastest_user_id: string | null;
  readonly fastest_contest_id: string | null;
  readonly fastest_submission_id: number | null;
  readonly execution_time: number | null;

  // Information for shortest code
  readonly shortest_user_id: string | null;
  readonly shortest_contest_id: string | null;
  readonly shortest_submission_id: number | null;
  readonly source_code_length: number | null;

  readonly solver_count: number | null;
  readonly point?: number | null;
}

export const isMergedProblem = (obj: unknown): obj is MergedProblem =>
  typeof obj === "object" &&
  obj !== null &&
  hasProperty(obj, "id") &&
  typeof obj.id === "string" &&
  hasProperty(obj, "contest_id") &&
  typeof obj.contest_id === "string" &&
  hasProperty(obj, "title") &&
  typeof obj.title === "string" &&
  hasProperty(obj, "first_user_id") &&
  (typeof obj.first_user_id === "string" || obj.first_user_id === null) &&
  hasProperty(obj, "first_contest_id") &&
  (typeof obj.first_contest_id === "string" || obj.first_contest_id === null) &&
  hasProperty(obj, "first_submission_id") &&
  (typeof obj.first_submission_id === "number" ||
    obj.first_submission_id === null) &&
  hasProperty(obj, "fastest_user_id") &&
  (typeof obj.fastest_user_id === "string" || obj.fastest_user_id === null) &&
  hasProperty(obj, "fastest_contest_id") &&
  (typeof obj.fastest_contest_id === "string" ||
    obj.fastest_contest_id === null) &&
  hasProperty(obj, "fastest_submission_id") &&
  (typeof obj.fastest_submission_id === "number" ||
    obj.fastest_submission_id === null) &&
  hasProperty(obj, "execution_time") &&
  (typeof obj.execution_time === "number" || obj.execution_time === null) &&
  hasProperty(obj, "shortest_user_id") &&
  (typeof obj.shortest_user_id === "string" || obj.shortest_user_id === null) &&
  hasProperty(obj, "shortest_contest_id") &&
  (typeof obj.shortest_contest_id === "string" ||
    obj.shortest_contest_id === null) &&
  hasProperty(obj, "shortest_submission_id") &&
  (typeof obj.shortest_submission_id === "number" ||
    obj.shortest_submission_id === null) &&
  hasProperty(obj, "source_code_length") &&
  (typeof obj.source_code_length === "number" ||
    obj.source_code_length === null) &&
  hasProperty(obj, "solver_count") &&
  (typeof obj.solver_count === "number" || obj.solver_count === null) &&
  (!hasProperty(obj, "point") ||
    typeof obj.point === "number" ||
    typeof obj.point === "undefined" ||
    obj.point === null);
