interface Response {
  status: 0 | 1,
  data: any,
  message: string,
}

export async function post(api: string, body: any) {
  let resp = await fetch(api, {
    method: 'post',
    body: JSON.stringify(body),
    headers: {
      'content-type': 'application/json',
    }
  }).then(resp => resp.json() as Promise<Response>);
  if (resp.status !== 0) {
    if (resp.message == 'auth error') {
      window.location.href = '/login';
    }
  }
  return resp;
}