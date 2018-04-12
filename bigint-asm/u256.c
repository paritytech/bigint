#include <stdint.h>

uint8_t u256add(uint64_t *first, uint64_t *second) {
  uint8_t overflow;

  asm (
       "addq %2, (%1)\n  "
       "adcq %3, 8(%1)\n  "
       "adcq %4, 16(%1)\n  "
       "adcq %5, 24(%1)\n  "
       "setc %0"
       : "=r"(overflow)
       : "r"(first),
         "r"(second[0]),
         "r"(second[1]),
         "r"(second[2]),
         "r"(second[3])
       );

  return overflow;
}

uint64_t u256mul(uint64_t *first, uint64_t *second, uint64_t *out) {
  uint64_t overflow;
  uint64_t result[4];

  // TODO: Could use movcc instead of jumpcc?
  asm (
    "mov %5, %%rax\n  "
    "mulq %9\n  "
    "mov %%rax, %1\n  "
    "mov %%rdx, %2\n  "

    "mov %5, %%rax\n  "
    "mulq %10\n  "
    "add %%rax, %2\n  "
    "adc $0, %%rdx\n  "
    "mov %%rdx, %3\n  "

    "mov %5, %%rax\n  "
    "mulq %11\n  "
    "add %%rax, %3\n  "
    "adc $0, %%rdx\n  "
    "mov %%rdx, %4\n  "

    "mov %5, %%rax\n  "
    "mulq %12\n  "
    "add %%rax, %4\n  "
    "adc $0, %%rdx\n  "
    "mov %%rdx, %0\n  "

    "mov %6, %%rax\n  "
    "mulq %9\n  "
    "add %%rax, %2\n  "
    "adc %%rdx, %3\n  "
    "adc $0, %4\n  "
    "adc $0, %0\n  "

    "mov %6, %%rax\n  "
    "mulq %10\n  "
    "add %%rax, %3\n  "
    "adc %%rdx, %4\n  "
    "adc $0, %0\n  "
    "adc $0, %4\n  "
    "adc $0, %0\n  "

    "mov %6, %%rax\n  "
    "mulq %11\n  "
    "add %%rax, %4\n  "
    "adc $0, %%rdx\n  "
    "or %%rdx, %0\n  "

    "mov %7, %%rax\n  "
    "mulq %9\n  "
    "add %%rax, %3\n  "
    "adc %%rdx, %4\n  "
    "adc $0, %0\n  "

    "mov %7, %%rax\n  "
    "mulq %10\n  "
    "add %%rax, %4\n  "
    "adc $0, %%rdx\n  "
    "or %%rdx, %0\n  "

    "mov %8, %%rax\n  "
    "mulq %9\n  "
    "add %%rax, %4\n  "
    "or %%rdx, %0\n  "

    "cmpq $0, %0\n  "
    "jne 2f\n  "

    "mov %8, %0\n  "
    "jrcxz 12f\n  "

    "mov %12, %0\n  "
    "mov %11, %%rax\n  "
    "or %%rax, %0\n  "
    "mov %10, %%rax\n  "
    "or %%rax, %0\n  "
    "jmp 2f\n  "

    "12:\n  "
    "mov %12, %0\n  "
    "jrcxz 11f\n  "

    "mov %7, %0\n  "
    "mov %6, %%rax\n  "
    "or %%rax, %0\n  "

    "cmpq $0, %0\n  "
    "jne 2f\n  "

    "11:\n  "
    "mov %11, %0\n  "
    "jrcxz 2f\n  "
    "mov %7, %0\n  "

    "2:\n  "
    : /* %0 */ "=r"(overflow),
      /* %1 */ "=&r"(result[0]),
      /* %2 */ "=&r"(result[1]),
      /* %3 */ "=&r"(result[2]),
      /* %4 */ "=&r"(result[3])
    : /* %5 */ "r"(first[0]),
      /* %6 */ "r"(first[1]),
      /* %7 */ "r"(first[2]),
      /* %8 */ "r"(first[3]),

      /* %9 */ "r"(second[0]),
      /* %10 */ "r"(second[1]),
      /* %11 */ "r"(second[2]),
      /* %12 */ "r"(second[3])
    : "rax", "rdx");

  out[0] = result[0];
  out[1] = result[1];
  out[2] = result[2];
  out[3] = result[3];

  return overflow;
}
