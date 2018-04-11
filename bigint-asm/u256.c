#include <stdint.h>

uint8_t u256add(
                      uint64_t *first,
                      uint64_t *second
                      ) {
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

uint64_t u256mul(
                      uint64_t *first,
                      uint64_t *second,
                      uint64_t *out
                      ) {
  register uint64_t overflow asm("rcx");
  uint64_t result0, result1, result2, result3;

  // TODO: Could use movcc instead of jumpcc?
  asm (
    "movq %4, %%rax\n  "
    "mulq %8\n  "
    "movq %%rax, %0\n  "
    "movq %%rdx, %1\n  "

    "movq %4, %%rax\n  "
    "mulq %9\n  "
    "addq %%rax, %1\n  "
    "adcq $0, %%rdx\n  "
    "movq %%rdx, %2\n  "

    "movq %4, %%rax\n  "
    "mulq %10\n  "
    "addq %%rax, %2\n  "
    "adcq $0, %%rdx\n  "
    "movq %%rdx, %3\n  "

    "movq %4, %%rax\n  "
    "mulq %11\n  "
    "addq %%rax, %3\n  "
    "adcq $0, %%rdx\n  "
    "movq %%rdx, %%rcx\n  "

    "movq %5, %%rax\n  "
    "mulq %8\n  "
    "addq %%rax, %1\n  "
    "adcq %%rdx, %2\n  "
    "adcq $0, %3\n  "
    "adcq $0, %%rcx\n  "

    "movq %5, %%rax\n  "
    "mulq %9\n  "
    "addq %%rax, %2\n  "
    "adcq %%rdx, %3\n  "
    "adcq $0, %%rcx\n  "
    "adcq $0, %3\n  "
    "adcq $0, %%rcx\n  "

    "movq %5, %%rax\n  "
    "mulq %10\n  "
    "addq %%rax, %3\n  "
    "adcq $0, %%rdx\n  "
    "orq %%rdx, %%rcx\n  "

    "movq %6, %%rax\n  "
    "mulq %8\n  "
    "addq %%rax, %2\n  "
    "adcq %%rdx, %3\n  "
    "adcq $0, %%rcx\n  "

    "movq %6, %%rax\n  "
    "mulq %9\n  "
    "addq %%rax, %3\n  "
    "adcq $0, %%rdx\n  "
    "orq %%rdx, %%rcx\n  "

    "movq %7, %%rax\n  "
    "mulq %8\n  "
    "addq %%rax, %3\n  "
    "orq %%rdx, %%rcx\n  "

    "cmpq $0, %%rcx\n  "
    "jne 2f\n  "

    "movq %7, %%rcx\n  "
    "jrcxz 12f\n  "

    "movq %11, %%rcx\n  "
    "movq %10, %%rax\n  "
    "orq %%rax, %%rcx\n  "
    "movq %9, %%rax\n  "
    "orq %%rax, %%rcx\n  "
    "jmp 2f\n  "

    "12:\n  "
    "movq %11, %%rcx\n  "
    "jrcxz 11f\n  "

    "movq %6, %%rcx\n  "
    "movq %5, %%rax\n  "
    "or %%rax, %%rcx\n  "

    "cmpq $0, %%rcx\n  "
    "jne 2f\n  "

    "11:\n  "
    "movq %10, %%rcx\n  "
    "jrcxz 2f\n  "
    "movq %6, %%rcx\n  "

    "2:\n  "
    : /* %0 */ "=&r"(result0),
      /* %1 */ "=&r"(result1),
      /* %2 */ "=&r"(result2),
      /* %3 */ "=&r"(result3)
    : /* %4 */ "r"(first[0]),
      /* %5 */ "r"(first[1]),
      /* %6 */ "r"(first[2]),
      /* %7 */ "r"(first[3]),

      /* %8 */ "r"(second[0]),
      /* %9 */ "r"(second[1]),
      /* %10 */ "r"(second[2]),
      /* %11 */ "r"(second[3])
    : "rax", "rdx", "rcx");

  out[0] = result0;
  out[1] = result1;
  out[2] = result2;
  out[3] = result3;

  return overflow;
}
