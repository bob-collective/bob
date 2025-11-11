import { KebabCase } from './types';
import kebabCase from 'lodash/fp/kebabCase';

export function mapByName<T extends { name: string }>(
  arr: T[],
): Record<KebabCase<T['name']>, T> {
  return arr.reduce((acc, object) => {
    acc[kebabCase(object.name)] = object;

    return acc;
  }, {} as Record<KebabCase<T['name']>, T>);
}
