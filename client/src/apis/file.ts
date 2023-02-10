import { AxiosProgressEvent } from "axios";
import path from "path-browserify";
import { post, post_formdata, post_raw, Response } from "./utils";

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

export async function create_dir(dir: string, file: string): Promise<boolean> {
  let resp = await post('/file/create_dir', {
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

export function create_compression_download_link(dir: string, file: string) {
  const url = new URL('/file/read_compression', window.location.origin);
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

export async function upload_file(dir: string, filename: string, file: File, config?: { onUploadProgress?: (e: AxiosProgressEvent) => void }) {
  const url = new URL('/file/upload', window.location.origin);
  const file_path = path.join(dir, filename);
  const form = new FormData();
  form.append('filename', file_path);
  form.append('file', file, filename);
  let resp = await post_formdata(url.toString(), form, config?.onUploadProgress);
  return resp;
}

export async function upload(dir: string, config?: { onUploadProgress: (e: AxiosProgressEvent) => void }): Promise<Response> {
  const input = document.createElement('input');
  input.setAttribute('type', 'file');
  input.style.display = 'none';
  document.body.appendChild(input);
  return new Promise((resolve, reject) => {
    input.addEventListener('change', async () => {
      let file = input.files?.item(0);
      if (file) {
        let resp = await upload_file(dir, file.name, file, config);
        resolve(resp);
      }
    }, false);
    input.click();
    document.body.removeChild(input);
  });
}