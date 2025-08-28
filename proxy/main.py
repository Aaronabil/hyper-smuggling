import httpx
from quart import Quart, request, Response

app = Quart(__name__)
# Alamat server backend di dalam network Docker
BACKEND_URL = "http://server:8080"

@app.route('/', defaults={'path': ''})
@app.route('/<path:path>', methods=['GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'HEAD', 'OPTIONS'])
async def proxy(path):
    # Aturan keamanan: Jangan biarkan siapapun mengakses /secret secara langsung
    if path == "secret":
        return Response("Forbidden: Access to this path is denied.", status=403)

    async with httpx.AsyncClient() as client:
        # Meneruskan request ke backend
        try:
            # Mengambil semua data dari request yang masuk
            headers = [(k, v) for k, v in request.headers.items()]
            data = await request.get_data()
            
            # Membuat request baru ke server backend
            backend_req = client.build_request(
                method=request.method,
                url=f"{BACKEND_URL}/{path}",
                headers=headers,
                content=data,
                params=request.args
            )
            
            # Mengirim request dan mendapatkan response
            backend_resp = await client.send(backend_req)

            # Meneruskan response dari backend ke client
            return Response(
                response=backend_resp.content,
                status=backend_resp.status_code,
                headers=backend_resp.headers.items()
            )
        except httpx.RequestError as e:
            return Response(f"Proxy error: {e}", status=502)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)