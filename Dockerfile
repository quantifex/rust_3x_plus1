FROM quantifex/rust_devtest:1_54

RUN apt-get update && apt-get install -y lcov

CMD ["/bin/bash"]