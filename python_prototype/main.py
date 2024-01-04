import helpers
from orchestradirector import OrchestraDirector


while True:
    message = input()

    command = message.split()[0]
    options = message[message.find(" ") + 1:]

    helpers.log_to_file(message)

    OrchestraDirector.handle_command(command, options)
