import axios, { AxiosProgressEvent } from 'axios';

export interface Response {
  status: 0 | 1,
  data: any,
  message: string,
}

const httpGroupHandlers = new Map<string, [AbortController, Promise<globalThis.Response>]>();

export async function post(api: string, body: any, tag = 'default') {
  let resp = await post_raw(api, body, tag).then(resp => resp.json() as Promise<Response>);
  if (resp.status !== 0) {
    if (resp.message === 'auth error') {
      window.location.href = '/login';
    }
  }
  return resp;
}

// unique request by tag
export async function post_raw(api: string, body: any, tag: string = 'default') {
  const handlers = httpGroupHandlers.get(tag);
  if (handlers) {
    return handlers[1].then((resp) => resp.clone());
  }
  const abort = new AbortController();
  let p = fetch(api, {
    method: 'post',
    signal: abort.signal,
    body: JSON.stringify(body),
    headers: {
      'content-type': 'application/json',
    }
  });
  httpGroupHandlers.set(tag, [abort, p]);
  let resp = await p;
  httpGroupHandlers.delete(tag);
  return resp.clone();
}

export async function post_formdata(api: string, body: FormData, onUploadProgress?: (e: AxiosProgressEvent) => void) {
  let resp = await axios.postForm(api, body, { responseType: 'json', onUploadProgress });
  return resp.data;
}


(window as any).__post = post;