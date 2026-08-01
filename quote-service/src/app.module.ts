import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { HealthModule } from './health/health.module';
import { QuotesModule } from './quotes/quotes.module';

@Module({
  imports: [HealthModule, QuotesModule],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
