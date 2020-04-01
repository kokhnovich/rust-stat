import binascii
import io
import logging
import stat
import sys
import threading
from typing import IO, Text

from absl import app
from absl import flags
from fleetspeak.server_connector.connector import InsecureGRPCServiceClient
from fleetspeak.src.common.proto.fleetspeak.common_pb2 import Message

from src.stat_pb2 import Request, Response, Error


FLAGS = flags.FLAGS

flags.DEFINE_string(
    name="client_id",
    default="",
    help="An id of the client to send the messages to.")

flags.DEFINE_string(
    name="output",
    default="",
    help="A path to the file to write the output to.")


def write_error(filedesc: IO[Text], response: Error):
    filedesc.write(f"{response.what}\n")

def write(filedesc: IO[Text], response: Response):
    filedesc.write(f"path: {response.path}\n")
    filedesc.write(f"size: {response.size} bytes\n")
    filedesc.write(f"mode: {stat.filemode(response.mode)}\n")
    filedesc.write(f"extra: {response.extra}\n")


def listener(message: Message, context):
    del context  # Unused

    kind = message.message_type
    if kind == "response":
        response = Response()
    else:
        response = Error()
    response.ParseFromString(message.data.value)

    if FLAGS.output:
        with io.open(FLAGS.output, mode="a", encoding="utf-8") as filedesc:
            if kind == "response":
                write(filedesc, response)
            else:
                write_error(filedesc, response)
    else:
        if kind == "response":
            write(sys.stdout, response)
        else:
            write_error(sys.stdout, response)


def main(argv=None):
    del argv  # Unused.

    service_client = InsecureGRPCServiceClient("stat")
    service_client.Listen(listener)

    while True:
        request = Request()
        request.path = input("Enter a path to stat: ")

        message = Message()
        message.destination.client_id = binascii.unhexlify(FLAGS.client_id)
        message.destination.service_name = "stater"
        message.data.Pack(request)

        service_client.Send(message)


if __name__ == "__main__":
    app.run(main)
