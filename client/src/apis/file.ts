import path from "path-browserify";
import { post } from "./utils";

export async function read_dir(dir: string) {
  let resp = await post('/file/read_dir', {
    file: dir
  });
  return resp.data.files;
}

export function create_download_link(dir: string, file: string) {
  const url = new URL('/file/read', 'http://localhost:7001');
  const file_path = path.join(dir, file);
  url.searchParams.set('file', file_path);
  return url.toString();
}