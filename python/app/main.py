''' Using flask to make an api '''
# import necessary libraries and functions
import uwsgidecorators
import os
import logging
from flask import Flask, jsonify, request
import helper
from sys import stdout
import requests
from opentelemetry.instrumentation.flask import FlaskInstrumentor
from opentelemetry.instrumentation.wsgi import OpenTelemetryMiddleware


from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.resources import SERVICE_NAME, Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor

OTEL_URL = os.environ['OTEL_EXPORTER_OTLP_ENDPOINT']


logging.basicConfig(level=logging.INFO, format="[%(asctime)s] %(name)-12s %(levelname)-8s %(filename)s:%(funcName)s %(message)s")

logFormatter = logging.Formatter("[%(asctime)s] %(name)-12s %(levelname)-8s %(filename)s:%(funcName)s %(message)s")


logger = logging.getLogger('werkzeug')
logger.setLevel(logging.INFO)
consoleHandler = logging.StreamHandler(stdout) #set streamhandler to stdout
consoleHandler.setFormatter(logFormatter)
logger.addHandler(consoleHandler)
config = helper.read_config()



resource = Resource(attributes={
SERVICE_NAME: "client",
"application.name": "client",
"env.name": "prod"
})



provider = TracerProvider(resource=resource)
processor = BatchSpanProcessor(OTLPSpanExporter(endpoint="http://opentelemetry-collector.monitoring.svc.cluster.local:4317"))
provider.add_span_processor(processor)
trace.set_tracer_provider(provider)



# tracer_provider = trace.get_tracer_provider()
# tracer = tracer_provider.get_tracer(__name__)
# # tracer.set_attribute(SERVICE_NAME, "your-service-name")
# processor = BatchSpanProcessor(OTLPSpanExporter(endpoint="http://simplest-collector.monitoring.svc.cluster.local:4318"))
# tracer_provider.add_span_processor(processor)
# # FlaskInstrumentor().instrument_app(flask)

app = Flask(__name__)
FlaskInstrumentor().instrument_app(app)
print ("asdf")
app.wsgi_app = OpenTelemetryMiddleware(app.wsgi_app, tracer_provider=provider)

from opentelemetry.instrumentation.requests import RequestsInstrumentor
RequestsInstrumentor().instrument()


# @tracer.start_as_current_span("get_server")
def get_foo():
    x = requests.get('http://foo1:5678')
    return x

@app.route('/', methods = ['GET', 'POST'])
# @tracer.start_as_current_span("client-home")
def home():
    if(request.method == 'GET'):
        logger.info('sample log')
        # carrier = {}
        # TraceContextTextMapPropagator().inject(carrier)
        # header = {"traceparent": carrier["traceparent"]}
        # logger.info(header)
        x = get_foo()
        logger.info(x.status_code)
        logger.info(x.text)
        data = f"Hello scaffold from {x.text}"
        return jsonify({'data': data,'lambda-response': x.text})
    return jsonify({'request': 'POST'})

@app.route('/home/<int:num>', methods = ['GET'])
def disp(num):
    return jsonify({'data': num**2})

@app.route('/health', methods = ['GET'])
def health():
    return jsonify({'status': 'healthy'})

if __name__ == '__main__':
    app.run(debug = True,  host="0.0.0.0", port = config['General']['port'])