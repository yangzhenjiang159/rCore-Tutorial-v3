# 设置默认的 Docker 镜像标签
DOCKER_TAG?= rcore-tutorial-v3:latest

# 定义伪目标，用于生成文档
.PHONY: docker build_docker

# 定义 docker 目标，用于在 Docker 容器中运行命令
docker:
	# 运行一个 Docker 容器，挂载当前目录到容器中的 /mnt 目录，设置工作目录为 /mnt，并指定容器名为 rcore-tutorial-v3
	docker run --rm -it -v ${PWD}:/mnt -w /mnt --name rcore-tutorial-v3 ${DOCKER_TAG} bash

# 定义 build_docker 目标，用于构建 Docker 镜像
build_docker: 
	# 使用当前目录下的 Dockerfile 构建一个名为 ${DOCKER_TAG} 的 Docker 镜像，并指定构建目标为 build
	docker build -t ${DOCKER_TAG} --target build.

# 定义 fmt 目标，用于格式化代码
fmt:
	# 进入 os 目录，执行 cargo fmt 命令格式化 Rust 代码，然后返回上一级目录
	cd os ; cargo fmt;  cd..
