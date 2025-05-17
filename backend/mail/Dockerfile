FROM gradle:jdk17 AS cache

WORKDIR /usr/src/minizord/mail

COPY gradle gradle
COPY build.gradle.kts settings.gradle.kts ./

RUN gradle dependencies --no-daemon --stacktrace

FROM gradle:jdk17 AS builder

WORKDIR /usr/src/minizord/mail

COPY --from=cache /usr/src/minizord/mail/.gradle ./.gradle
COPY --from=cache /usr/src/minizord/mail .

COPY src src

RUN gradle bootJar -x test --no-daemon --stacktrace

FROM openjdk:17-jdk

WORKDIR /minizord
COPY --from=builder /usr/src/minizord/mail/build/libs/*.jar ./minizord-mail.jar

CMD ["java", "-jar", "minizord-mail.jar"]