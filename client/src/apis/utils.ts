import axios, { AxiosProgressEvent } from 'axios';

export interface Response {
  status: 0 | 1,
  data: any,
  message: string,
}

const httpGroupHandlers = new Map<string, AbortController[]>();

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
  const abort = new AbortController();
  const handlers = httpGroupHandlers.get(tag) || [];
  while (handlers.length) {
    handlers.pop()?.abort();
  }
  handlers.push(abort);
  httpGroupHandlers.set(tag, handlers);
  let resp = await fetch(api, {
    method: 'post',
    signal: abort.signal,
    body: JSON.stringify(body),
    headers: {
      'content-type': 'application/json',
    }
  })
  let idx = handlers.findIndex(i => i === abort);
  if (idx !== -1) {
    handlers.splice(idx, 1);
  }
  return resp;
}

export async function post_formdata(api: string, body: FormData, onUploadProgress?: (e: AxiosProgressEvent) => void) {
  let resp = await axios.postForm(api, body, { responseType: 'json', onUploadProgress });
  return resp.data;
}
