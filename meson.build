project('ya_raytracer_rt', ['cpp'], license : '0BSD', default_options : ['cpp_std=c++23'])

inc = include_directories('include')

executable('main', 'src/main.cpp', include_directories : inc)

cuda = import('unstable-cuda')
cuda.nvcc_arch_flags('Ampere')
