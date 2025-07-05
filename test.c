#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#define NUM_THREADS (400)
#define MAX_ALLOCATION_SIZE_BYTES (1 * 1024 * 1024)
#define PAGE_SIZE_BYTES (4096)

void* runThread(void* param) {
  int i;
  int pageBytes;
  void* pData;

  while (true) {
    size_t bytes = lrand48() % MAX_ALLOCATION_SIZE_BYTES;
    if (bytes == 0) {
       bytes = 1;
    }
    pData = malloc(bytes);
    if (pData == NULL) {
        printf("malloc returned NULL\n");
        abort();
    }
    for (pageBytes = 0;
         pageBytes < bytes;
         pageBytes += PAGE_SIZE_BYTES) {
      unsigned char* pByteArray = pData;
      pByteArray[pageBytes] = 0;
    }
    /*memset(pData, 0, ALLOCATION_SIZE_BYTES);*/
    free(pData);
  }

  return NULL;
}

int main(int argc, char** arg) {
  int i;
  int retVal;
  pthread_t* threadIDs;

  printf("begin main\n");

  printf("creating %d threads\n", NUM_THREADS);

  threadIDs = calloc(NUM_THREADS, sizeof(pthread_t));

  for (i = 0; i < NUM_THREADS; ++i) {
    retVal = pthread_create(&threadIDs[i], NULL, &runThread, NULL);
    if (retVal != 0) {
      printf("pthread_create returned NULL\n");
      abort();
    }
  }

  printf("joining %d threads\n", NUM_THREADS);

  for (i = 0; i < NUM_THREADS; ++i) {
    retVal = pthread_join(threadIDs[i], NULL);
    if (retVal != 0) {
      printf("pthread_join returned NULL\n");
      abort();
    }
  }

  printf("end main\n");
  return 0;
}