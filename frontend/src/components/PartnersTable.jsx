import PartnersTableRow from "./PartnersTableRow";

function PartnersTable({ partners, onEdit }) {
  return (
    <table className="w-full border-collapse">
      <thead className="bg-gray-100">
        <tr>
          <th className="p-3 text-left">Name</th>
          <th className="p-3 text-left">Email</th>
          <th className="p-3 text-left">Website</th>
          <th className="p-3 text-left">Countries</th>
          <th className="p-3 text-left">Actions</th>
        </tr>
      </thead>
      <tbody>
        {partners.map((p) => (
          <PartnersTableRow key={p.id} partner={p} onEdit={onEdit} />
        ))}
      </tbody>
    </table>
  );
}

export default PartnersTable;
