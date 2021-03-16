FROM amd64/alpine:edge as builder
RUN apk add --no-cache build-base tzdata curl

COPY docker/arch docker/config.mak .

ARG TARGETPLATFORM
RUN [ $(awk -v target_arch=${TARGETPLATFORM} '$1==target_arch {print $4}' arch) ] && \
	apk add --no-cache git file cmake && \
	git clone https://github.com/richfelker/musl-cross-make --branch v0.9.8 && \
	cp config.mak musl-cross-make && \
	cd musl-cross-make && \
	awk -v target_arch=${TARGETPLATFORM} '$1==target_arch {print "\nTARGET = " $4}' ../arch >> config.mak && \
	make install -j 4 && \
	ln -s /usr/local/musl/bin/$(awk -v target_arch=${TARGETPLATFORM} '$1==target_arch {print $4}' ../arch)-strip \
	/usr/local/musl/bin/musl-strip && \
	cd && \
	rm -rf /tmp/musl-cross-make \
	|| cd 

ENV PATH=$PATH:/usr/local/musl/bin:/root/.cargo/bin

RUN chmod 755 /root/ && \
    curl https://sh.rustup.rs -sqSf | \
    sh -s -- -y --default-toolchain stable && \
    rustup target add $(awk -v target_arch=${TARGETPLATFORM} '$1==target_arch {print $2}' arch)

RUN TARGET=$(awk -v target_arch=${TARGETPLATFORM} '$1==target_arch {print $2}' arch) && \
    TARGET_LINKER=$(awk -v target_arch=${TARGETPLATFORM} '$1==target_arch {print $3}' arch) && \
	echo -e "[build]\ntarget = \"$TARGET\"\n\n[target.$TARGET]\nlinker = \"$TARGET_LINKER\"\n" > /root/.cargo/config

ENV CC_arm-unknown-linux-musleabihf=/usr/local/musl/bin/armv6-linux-musleabihf-gcc
ENV CC_armv7-unknown-linux-musleabihf=/usr/local/musl/bin/armv7-linux-musleabihf-gcc
ENV CC_aarch64-unknown-linux-musl=/usr/local/musl/bin/aarch64-linux-musl-gcc
ENV CC_x86_64-unknown-linux-musl=gcc

COPY . .
RUN cargo install --path . --target $(awk -v target_arch=${TARGETPLATFORM} '$1==target_arch {print $2}' arch)

FROM alpine:latest
RUN apk add --no-cache tzdata
COPY --from=builder /root/.cargo/bin/tf-viewer /bin/tf-viewer

WORKDIR /data
VOLUME ["/data"]
EXPOSE 8080
ENTRYPOINT ["/bin/tf-viewer"]
