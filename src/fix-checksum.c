#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <errno.h>
#include <stdio.h>
#include <fcntl.h>
#include <sys/mman.h>

static int
error(const char *self, const char *binary, const char *err)
{
	fprintf(stderr, "%s: %s: %s\n", self, binary, err);
	return EXIT_FAILURE;
}

int
main(int argc, char *argv[])
{
	int fd;
	void *map;
	uint32_t *data;
	uint32_t sum = 0;
	int i;

	if (argc != 2) {
		fprintf(stderr, "%s: pass a binary filename please\n", argv[0]);
		return 1;
	}

	fd = open(argv[1], O_RDWR);
	if (fd < 0)
		return error(argv[0], argv[1], strerror(errno));

	map = mmap(NULL, 0x20, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
	if (map == MAP_FAILED)
		return error(argv[0], argv[1], strerror(errno));
	data = map;

	for (i = 0; i < 7; i++)
		sum += data[i];
	data[i] = -sum;

	return EXIT_SUCCESS;
}
