

CFLAGS := -Wall
SOURCES := main.c vec3.c ray.c
TARGET := main

all:
	$(CC) $(CFLAGS) $(SOURCES) -o $(TARGET)
