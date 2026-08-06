import {
  Injectable,
  Logger,
  OnModuleDestroy,
  OnModuleInit,
} from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import {
  Pool,
  PoolClient,
  PoolConfig,
  QueryResult,
  QueryResultRow,
} from 'pg';

@Injectable()
export class DatabaseService implements OnModuleInit, OnModuleDestroy {
  private readonly logger = new Logger(DatabaseService.name);
  private readonly pool: Pool;

  constructor(private readonly configService: ConfigService) {
    const connectionString = this.configService.get<string>('DATABASE_URL');

    const poolConfig: PoolConfig = connectionString
      ? { connectionString }
      : {
          host: this.configService.getOrThrow<string>('DATABASE_HOST'),
          port: Number(
            this.configService.getOrThrow<string>('DATABASE_PORT'),
          ),
          database: this.configService.getOrThrow<string>('DATABASE_NAME'),
          user: this.configService.getOrThrow<string>('DATABASE_USER'),
          password: this.configService.getOrThrow<string>('DATABASE_PASSWORD'),
        };

    this.pool = new Pool({
      ...poolConfig,
      max: 10,
      idleTimeoutMillis: 30_000,
      connectionTimeoutMillis: 5_000,
    });

    this.pool.on('error', (error: Error) => {
      this.logger.error('Unexpected PostgreSQL pool error', error.stack);
    });
  }

  async onModuleInit(): Promise<void> {
    await this.pool.query('SELECT 1');
    this.logger.log('PostgreSQL connection established');
  }

  async onModuleDestroy(): Promise<void> {
    await this.pool.end();
    this.logger.log('PostgreSQL connection pool closed');
  }

  query<T extends QueryResultRow = QueryResultRow>(
    text: string,
    values: readonly unknown[] = [],
  ): Promise<QueryResult<T>> {
    return this.pool.query<T>(text, [...values]);
  }

  connect(): Promise<PoolClient> {
    return this.pool.connect();
  }
}
