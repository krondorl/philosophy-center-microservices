/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

import { Injectable } from '@nestjs/common';

@Injectable()
export class AppService {
  getHello(): string {
    return 'Hello World!';
  }
}
