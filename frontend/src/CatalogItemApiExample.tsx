import { useEffect, useState } from 'react'
import { client, listCatalogItems } from '@repo/catalog-client'
import type { CatalogItem } from '@repo/catalog-client'
import './CatalogItemApiExample.css'

client.setConfig({ baseUrl: '' });

export default function CatalogItemApiExample() {
  const [items, setItems] = useState<CatalogItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    listCatalogItems({ query: { maxResults: 50 } })
      .then(({ data, error }) => {
        if (error || !data) {
          setError('Failed to load catalog items');
        } else {
          setItems(data.items);
        }
      })
      .catch(() => setError('Failed to connect to server'))
      .finally(() => setLoading(false));
  }, []);

  return (
    <div className="catalog-example">
      <h2>Catalog Items</h2>

      {loading && <p className="status">Loading...</p>}
      {error && <p className="status error">{error}</p>}

      {!loading && !error && items.length === 0 && (
        <p className="status">No items found.</p>
      )}

      {items.length > 0 && (
        <table className="catalog-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Category</th>
              <th>Brand</th>
              <th>Price</th>
              <th>Date</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody>
            {items.map((item) => (
              <tr key={item.itemId}>
                <td>{item.name}</td>
                <td><span className="badge">{item.category}</span></td>
                <td>{item.brand ?? '—'}</td>
                <td className="price">${item.price}</td>
                <td className="date">{item.date}</td>
                <td className="description">{item.description}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  )
}
