project('ya_raytracer_rt', ['cuda', 'cpp'], license : '0BSD', default_options : ['cpp_std=c++17'])

inc = include_directories('include')

executable('main', 'main.cpp', include_directories : inc)

cuda = import('unstable-cuda')
cuda.nvcc_arch_flags('Ampere')
