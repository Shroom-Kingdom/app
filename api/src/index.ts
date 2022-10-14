import { router } from './main';

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    if (request.method === 'OPTIONS') {
      const headers = new Headers();
      setupCORS(request, headers);
      return new Response('', { status: 204, headers });
    }
    try {
      const res = await router.handle(request, env);
      const response = new Response(res.body, res);
      setupCORS(request, response.headers);
      return response;
    } catch (e) {
      const headers = new Headers();
      setupCORS(request, headers);
      if (e instanceof Error) {
        console.log('Internal Error', e.message);
        return new Response(e.message, { status: 500, headers });
      }
      console.log('Internal Error', e);
      return new Response('Unknown Error', { status: 500, headers });
    }
  }
};

function setupCORS(request: Request, headers: Headers) {
  const origin = request.headers.get('Origin');
  if (origin != null) {
    headers.set('Access-Control-Allow-Origin', origin);
  }
}
