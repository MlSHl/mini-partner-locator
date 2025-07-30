function PartnersTableRow({ partner, onEdit }) {

  return (
    <tr className="border-t">
      <td className="p-3">{partner.name}</td>
      <td className="p-3">{partner.email}</td>
      <td className="p-3">{partner.website_url}</td>
      <td className="p-3">
        {(partner.countries || []).map((c) => c.name).join(", ")}
      </td>
      <td className="p-3">
        <button
          className="text-blue-600 hover:underline"
          onClick={() => onEdit(partner)}
        >
          Edit
        </button>
      </td>
    </tr>
  );
}

export default PartnersTableRow;
