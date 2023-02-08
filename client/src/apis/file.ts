import path from "path-browserify";
import { post, post_raw } from "./utils";

export interface FileStat {
  name: string;
  size: number;
  is_dir: boolean;
  is_file: boolean;
  created: number;
  modified: number;
  accessed: number;
}

export async function read_dir(dir: string): Promise<FileStat[]> {
  let resp = await post('/file/read_dir', {
    file: dir
  });
  return resp.data.files;
}

export async function delete_file(dir: string, file: string): Promise<boolean> {
  let resp = await post('/file/delete', {
    file: path.join(dir, file)
  });
  return resp.status === 0;
}

export function create_download_link(dir: string, file: string) {
  const url = new URL('/file/read', window.location.origin);
  const file_path = path.join(dir, file);
  url.searchParams.set('file', file_path);
  return url.toString();
}

export async function read_text_file(dir: string, file: string) {
  const url = new URL('/file/read', window.location.origin);
  const file_path = path.join(dir, file);
  let resp = await post_raw(url.toString(), { file: file_path });
  let content = await resp.text();
  return content;
}