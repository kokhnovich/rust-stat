from absl import app
from fleetspeak.src.common.proto.fleetspeak.common_pb2 import Message
from fleetspeak.client_connector.connector import FleetspeakConnection

from stat_pb2 import Request, Response


def main(argv):
    del argv  # Unused.

    conn = FleetspeakConnection(version="0.0.1")
    while True:
        message, _ = conn.Recv()

        request = Request()
        message.data.Unpack(request)

        # TODO: Provide a real response.
        response = Response()
        response.path = request.path
        response.size = 1337
        response.mode = 0o1666

        message = Message()
        message.destination.service_name = "stat"
        message.data.Pack(response)

        conn.Send(message)


if __name__ == "__main__":
    app.run(main)
