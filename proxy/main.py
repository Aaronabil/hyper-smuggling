from flask import Flask, request, Response
import requests
import os

app = Flask(__name__)
BACKEND_URL = os.environ.get('BACKEND_URL', 'http://localhost:3000')

@app.route('/', defaults={'path': ''}, methods=['GET', 'POST', 'PUT', 'DELETE', 'HEAD', 'OPTIONS'])
@app.route('/<path:path>', methods=['GET', 'POST', 'PUT', 'DELETE', 'HEAD', 'OPTIONS'])
def proxy(path):
    # Simple proxy that forwards requests
    if path:
        url = f"{BACKEND_URL}/{path}"
    else:
        url = f"{BACKEND_URL}/"
    
    # Forward headers (but normalize them)
    headers = {}
    for key, value in request.headers:
        if key.lower() not in ['host', 'content-length', 'connection']:
            headers[key] = value
    
    # Forward the request
    try:
        resp = requests.request(
            method=request.method,
            url=url,
            headers=headers,
            data=request.get_data(),
            params=request.args,
            allow_redirects=False
        )
        
        # Return response
        response_headers = {}
        for key, value in resp.headers.items():
            if key.lower() not in ['content-encoding', 'transfer-encoding', 'connection']:
                response_headers[key] = value
        
        response = Response(
            resp.content,
            status=resp.status_code,
            headers=response_headers
        )
        return response
    except Exception as e:
        return f"Proxy Error: {str(e)}", 500

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8080, debug=False, threaded=True)