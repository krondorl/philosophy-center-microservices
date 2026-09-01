/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

import { Logger } from '@nestjs/common';
import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  await app.listen(process.env.PORT ?? 3232);

  const logger = new Logger();

  logger.log('');
  logger.log('Quote Service started');
  logger.log(
    `Server started at http://${process.env.SERVER_ADDRESS}:${process.env.SERVER_PORT}`,
  );
}
bootstrap();
