import socket

def send_and_receive(message, host='127.0.0.1', port=7777):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    
    try:
        sock.connect((host, port))
        
        sock.sendall(message.encode('utf-8'))
        
        # Shutdown sending side to signal EOF
        sock.shutdown(socket.SHUT_WR)
        
        # Receive response
        data = sock.recv(64)
        if data:
            print(f"Received: {data.decode('utf-8')}")
        else:
            print("No response received.")
            
    except Exception as e:
        print(f"Error: {e}")
    finally:
        sock.close()

def main():
    while True:
        message = input("Enter message (or 'quit' to exit): ")
        if message.lower() == 'quit':
            break
        if message:
            send_and_receive(message)

if __name__ == "__main__":
    main()
